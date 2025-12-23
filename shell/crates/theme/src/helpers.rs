use palette::Oklcha;

/// Convert "oklch(...)" / "oklcha(...)" into `palette::Oklcha`
pub fn parse_oklcha_str(s: &str) -> Result<Oklcha, String> {
    let input = s.trim();
    let lower = input.to_lowercase();

    // strip "oklch(" or "oklcha("
    let body = if let Some(idx) = lower.find("oklch(") {
        &input[idx + "oklch(".len()..]
    } else if let Some(idx) = lower.find("oklcha(") {
        &input[idx + "oklcha(".len()..]
    } else {
        return Err("expected oklch(...) or oklcha(...)".into());
    };

    let body = body.strip_suffix(')').ok_or("missing closing ')'")?.trim();

    // split alpha via `/`
    let (main, alpha_part) = if let Some(idx) = body.find('/') {
        (body[..idx].trim(), Some(body[idx + 1..].trim()))
    } else {
        (body, None)
    };

    let t = main.replace(',', " ");
    // tokenize L C H
    let tokens: Vec<&str> = t.split_whitespace().collect();
    if tokens.len() < 3 {
        return Err("expected at least 3 components: L C H".into());
    }

    let l = parse_number_or_percent(tokens[0])?;
    let c = tokens[1].parse::<f32>().map_err(|e| e.to_string())?;
    let h = parse_hue(tokens[2])?;

    // alpha
    let a = if let Some(alpha) = alpha_part {
        parse_number_or_percent(alpha)?
    } else if tokens.len() >= 4 {
        parse_number_or_percent(tokens[3])?
    } else {
        1.0
    };

    Ok(Oklcha::new(l, c, h, a))
}

// ------------------ Helpers ------------------

fn parse_number_or_percent(s: &str) -> Result<f32, String> {
    let s = s.trim();
    if let Some(num) = s.strip_suffix('%') {
        let v: f32 = num
            .trim()
            .parse()
            .map_err(|e: std::num::ParseFloatError| e.to_string())?;
        Ok(v / 100.0)
    } else {
        s.parse::<f32>().map_err(|e| e.to_string())
    }
}

fn parse_hue(s: &str) -> Result<f32, String> {
    let s = s.trim().to_lowercase();

    if let Some(x) = s.strip_suffix("deg") {
        return x
            .trim()
            .parse::<f32>()
            .map_err(|e: std::num::ParseFloatError| e.to_string());
    }
    if let Some(x) = s.strip_suffix("rad") {
        let v: f32 = x
            .trim()
            .parse()
            .map_err(|e: std::num::ParseFloatError| e.to_string())?;
        return Ok(v.to_degrees());
    }
    if let Some(x) = s.strip_suffix("grad") {
        let v: f32 = x
            .trim()
            .parse()
            .map_err(|e: std::num::ParseFloatError| e.to_string())?;
        return Ok(v * 0.9);
    }
    if let Some(x) = s.strip_suffix("turn") {
        let v: f32 = x
            .trim()
            .parse()
            .map_err(|e: std::num::ParseFloatError| e.to_string())?;
        return Ok(v * 360.0);
    }

    // bare number = degrees
    s.parse::<f32>().map_err(|e| e.to_string())
}
