import { Helpers } from "./helpers";

// Type aliases for simplification.
type ClassInstance = object;
type StaticClass = (new () => never);
type Class = ClassInstance | StaticClass;

/** Specifies the log level of log messages. */
enum LogLevel {
  ERROR = 1,
  WARN = 2,
  INFO = 3,
  DEBUG = 4,
}

/** Replacement for console. */
export class Logger {
  static logLevel: LogLevel =
    process.env.NODE_ENV === "test" ? LogLevel.WARN : LogLevel.INFO;

  static as(name: string): LoggerInstance {
    return new LoggerInstance(name);
  }

  static debug(c: Class, message: unknown, ...rest: unknown[]): void {
    if (Logger.logLevel >= LogLevel.DEBUG)
      Logger.asClass(c).debug(message, ...rest);
  }

  static info(c: Class, message: unknown, ...rest: unknown[]): void {
    if (Logger.logLevel >= LogLevel.INFO)
      Logger.asClass(c).info(message, ...rest);
  }

  static warn(c: Class, message: unknown, ...rest: unknown[]): void {
    if (Logger.logLevel >= LogLevel.WARN)
      Logger.asClass(c).warn(message, ...rest);
  }

  static error(c: Class, message: unknown, ...rest: unknown[]): void {
    Logger.asClass(c).error(message, ...rest);
  }

  static print(...messages: unknown[]): void {
    console.log(...messages);
  }

  private static asClass(c: Class): LoggerInstance {
    const isTypeInstance =
      typeof c === "object" && c.constructor.name !== "Object";
    const name = isTypeInstance ? c.constructor.name : (c as StaticClass).name;
    return new LoggerInstance(name);
  }

  static setDebugLevel(): void {
    Logger.logLevel = LogLevel.DEBUG;
  }

  static setInfoLevel(): void {
    Logger.logLevel = LogLevel.INFO;
  }

  static setWarningLevel(): void {
    Logger.logLevel = LogLevel.WARN;
  }

  static setErrorLevel(): void {
    Logger.logLevel = LogLevel.ERROR;
  }
}

/** Logger instance. */
export class LoggerInstance {
  constructor(private name: string) {}

  debug(message: unknown, ...rest: unknown[]): void {
    if (Logger.logLevel >= LogLevel.DEBUG)
      console.log(this.format("DEBUG"), message, ...rest);
  }

  info(message: unknown, ...rest: unknown[]): void {
    if (Logger.logLevel >= LogLevel.INFO)
      console.log(this.format("INFO"), message, ...rest);
  }

  warn(message: unknown, ...rest: unknown[]): void {
    if (Logger.logLevel >= LogLevel.WARN)
      console.warn(this.format("WARN"), message, ...rest);
  }

  error(message: unknown, ...rest: unknown[]): void {
    console.error(this.format("ERROR"), message, ...rest);
  }

  private format(level: string): string {
    return `| ${Helpers.formatTimeWithMillis()} | ${level} | ${this.name} |`;
  }
}
