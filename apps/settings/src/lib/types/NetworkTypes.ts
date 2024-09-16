export interface KnownNetworkResponse {
    network_id: string,
    ssid: string,
    flags: string,
}
export interface KnownNetworkListResponse {
    known_network: KnownNetworkResponse[],
}

export interface WirelessInfoResponse {
    mac: string,
    frequency: string,
    signal: string,
    flags: string,
    name: string,
    security?: string,
    encryption?: string,
    isSecured?: boolean
}
export interface WirelessScanListResponse {
    wireless_network: WirelessInfoResponse[],
}

