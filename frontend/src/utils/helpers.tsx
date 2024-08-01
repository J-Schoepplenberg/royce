/** Examplatory helper functions. */
export class Helpers {
  static formatTimeWithMillis(time: number | Date = Date.now()): string {
    const { year, month, day, hour, minute, second, millis, timeZone } =
      Helpers.parseDate(time);
    return `${year}-${month}-${day} ${hour}:${minute}:${second}.${millis} ${timeZone}`;
  }

  static parseDate(time: number | Date = Date.now()): {
    year: string;
    month: string;
    day: string;
    hour: string;
    minute: string;
    second: string;
    millis: string;
    timeZone: string;
  } {
    const date = Helpers.isNumber(time) ? new Date(time) : time;
    const year = String(date.getFullYear());
    const month = Helpers.pad(date.getMonth() + 1);
    const day = Helpers.pad(date.getDate());
    const hour = Helpers.pad(date.getHours());
    const minute = Helpers.pad(date.getMinutes());
    const second = Helpers.pad(date.getSeconds());
    const millis = Helpers.pad(date.getMilliseconds(), 3);
    const offset = date.getTimezoneOffset();
    const timeZone =
      offset === 0 ? "UTC" : `UTC${Helpers.formatSign(-offset / 60)}`;
    return { year, month, day, hour, minute, second, millis, timeZone };
  }

  static isNumber = (value: unknown): value is number => Number.isFinite(value);

  static formatSign(value: number): string {
    const sign = value >= 0 ? "+" : "";
    return `${sign}${value}`;
  }

  static pad = (value: number, length = 2) =>
    value.toString().padStart(length, "0");
}
