/**
 * Configuration file source with path normalization.
 *
 * Path: xiaoyi.core.config.source.file
 *
 * Layer hierarchy:
 * - 0: core
 * - 1: config
 * - 2: source
 * - 3: file
 * - 4: path/absolute/unix/norm
 */

export interface FileSourceConfig {
  path: string;
  required?: boolean;
}

export class FileSource {
  readonly path: string;
  readonly required: boolean;

  constructor(config: FileSourceConfig) {
    this.path = config.path;
    this.required = config.required ?? true;
  }

  static new(path: string): FileSource {
    return new FileSource({ path });
  }

  optional(): FileSource {
    this.required = false;
    return this;
  }

  async load(): Promise<Record<string, unknown>> {
    const fs = await import("fs/promises");
    const path = await import("path");

    try {
      const content = await fs.readFile(this.path, "utf-8");
      const ext = path.extname(this.path).toLowerCase().slice(1);

      switch (ext) {
        case "toml": {
          const toml = await import("toml");
          return toml.parse(content);
        }
        case "json":
          return JSON.parse(content);
        case "yaml":
        case "yml": {
          const yaml = await import("yaml");
          return yaml.parse(content) ?? {};
        }
        default:
          throw new Error(`[config] unsupported config file format: ${ext}`);
      }
    } catch (err) {
      if (this.required) {
        throw new Error(`[config] failed to load config: ${err}`);
      }
      return {};
    }
  }
}

// Path normalization utilities
export namespace norm {
  export function normalize(input: string): string {
    const parts = input.split("/").filter((p) => p !== "");
    const result: string[] = [];
    for (const part of parts) {
      if (part === "..") {
        if (result.length > 0 && result[result.length - 1] !== "..") {
          result.pop();
        } else {
          result.push(part);
        }
      } else if (part !== ".") {
        result.push(part);
      }
    }
    return result.join("/") || ".";
  }

  export function absolute(input: string): string {
    return path.isAbsolute(input) ? input : path.join(process.cwd(), input);
  }

  export function unix(input: string): string {
    return input.replace(/\\/g, "/");
  }
}