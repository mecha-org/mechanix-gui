export const goBack = () => {
	window.history.back();
};


export enum LOG_LEVEL {
	'ERROR',
	'INFO',
	'WARNING',
	'LOG'
}
export interface ILog {
	data?: any,
	type?: LOG_LEVEL
}


export const consoleLog = (message: string, log?: ILog) => {

	if (!log) {
		console.log(message);
		return;
	}

	const { type, data } = log;
	switch (type) {
		case LOG_LEVEL.ERROR:
			console.error(message, data);
			break;
		case LOG_LEVEL.INFO:
			console.info(message, data);
			break;
		case LOG_LEVEL.WARNING:
			console.warn(message, data);
			break;

		default:
			console.log(message, data);
			break;
	}
}