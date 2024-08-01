import { Logger } from "./logging";

/** HTTP requests from frontend to backend. */
export class Requests {
  /** Fetch some URL path asynchronously. */
  private static async fetch(
    path: string,
    options: RequestInit
  ): Promise<Response> {
    try {
      const response = await fetch(path, options);
      if (!response.ok) {
        Logger.error(this, `Request to ${path} failed:`, response, options);
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return response;
    } catch (error) {
      Logger.error(this, `Request to ${path} failed:`, error, options);
      throw new Error(`Failed to fetch: ${error}`);
    }
  }

  /** GET some JSON. */
  static async getJson(path: string): Promise<unknown> {
    Logger.debug(this, `GET ${path}`);
    const response = await Requests.fetch(path, {
      method: "GET",
      headers: { credentials: "include" },
    });
    try {
      const json = await response.json();
      Logger.debug(this, `GET ${path} response:`, json);
      return json;
    } catch (error) {
      Logger.debug(this, `GET ${path} response:`, response);
      throw new Error(`Failed to parse JSON: ${error}`);
    }
  }

  /** POST some JSON. */
  static async postJson(path: string, params: unknown): Promise<unknown> {
    Logger.debug(this, `POST ${path} request params:`, params);
    const response = await Requests.fetch(path, {
      method: "POST",
      headers: { "Content-Type": "application/json", credentials: "include" },
      body: JSON.stringify(params),
    });
    try {
      const json = await response.json();
      Logger.debug(this, `POST ${path} response:`, json);
      return json;
    } catch (error) {
      Logger.debug(this, `POST ${path} response:`, response);
      throw new Error(`Failed to parse JSON: ${error}`);
    }
  }
}
