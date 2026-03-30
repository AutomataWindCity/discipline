export class TextualErrorAttachment {
  private constructor(
    readonly name: string,
    readonly value: string,
  ) {}

  static create(
    name: string,
    value: string,
  ) {
    return new TextualErrorAttachment(name, value);
  }

  static is(it: unknown): it is TextualErrorAttachment {
    return it instanceof TextualErrorAttachment;
  }
}

export class TextualErrorContext {
  private constructor(
    readonly action: string,
    readonly messages: string[],
    readonly attachements: TextualErrorAttachment[],
  ) {}

  static create(
    action: string,
  ) {
    return new TextualErrorContext(action, [], []);
  }

  static is(it: unknown): it is TextualErrorContext {
    return it instanceof TextualErrorContext;
  }
}

export class TextualError {
  private constructor(
    private context: TextualErrorContext,
    readonly earlierContexts: TextualErrorContext[],
  ) {}

  static create(action: string) {
    return new TextualError(
      TextualErrorContext.create(action),
      [],
    );
  }

  static is(it: unknown): it is TextualError {
    return it instanceof TextualError;
  }

  changeContext(newContextAction: string): TextualError {
    this.earlierContexts.push(this.context);
    this.context = TextualErrorContext.create(newContextAction);
    return this;
  }

  addMessage(message: string): TextualError {
    this.context.messages.push(message);
    return this;
  }

  addStringAttachment(name: string, value: string) {
    this.context.attachements.push(TextualErrorAttachment.create(
      name,
      `"${value.replaceAll(/"/g, '\\"').replaceAll(/\n/g, '\\n')}"`
    ));
    return this;
  }
  
  addErrorAttachment(name: string, value: Error) {
    this.context.attachements.push(TextualErrorAttachment.create(
      name,
      `${value}`
    ));
    return this;
  }

  addNumberAttachment(name: string, value: number) {
    this.context.attachements.push(TextualErrorAttachment.create(
      name,
      value.toString(),
    ));
    return this;
  }

  addBooleanAttachment(name: string, value: boolean) {
    this.context.attachements.push(TextualErrorAttachment.create(
      name,
      value ? "true" : "false",
    ));
    return this;
  }

  addNullAttachment(name: string) {
    this.context.attachements.push(TextualErrorAttachment.create(
      name,
      "null",
    ));
    return this;
  }

  addPrimitiveAttachment(name: string, value: null | string | number | boolean): TextualError {
    if (value === null) {
      return this.addNullAttachment(name);
    }
    switch (typeof value) {
      case "string": {
        return this.addStringAttachment(name, value);
      }
      case "number": {
        return this.addNumberAttachment(name, value);
      }
      case "boolean": {
        return this.addBooleanAttachment(name, value);
      }
    }
  }

  print(options?: { color?: boolean }): string {
    const useColor = options?.color ?? false;

    const c = {
      reset: useColor ? "\x1b[0m" : "",
      dim: useColor ? "\x1b[2m" : "",
      red: useColor ? "\x1b[31m" : "",
      yellow: useColor ? "\x1b[33m" : "",
      cyan: useColor ? "\x1b[36m" : "",
      gray: useColor ? "\x1b[90m" : "",
    };

    const lines: string[] = [];

    const branch = "│";
    const tee = "├─";
    const last = "└─";

    const formatStack = (value: string, prefix: string): string[] => {
      if (!value.includes("\n")) return [`${prefix}${c.gray}${value}${c.reset}`];

      return value.split("\n").map((line, i) =>
        i === 0
          ? `${prefix}${c.gray}${line}${c.reset}`
          : `${prefix}${branch}  ${c.dim}${line}${c.reset}`
      );
    };

    const formatContext = (
      ctx: TextualErrorContext,
      prefix: string,
      isLast: boolean,
      index?: number
    ): string[] => {
      const ctxLines: string[] = [];

      const head = isLast ? last : tee;
      const label = index === undefined
        ? `${c.red}💥 ${ctx.action}${c.reset}`
        : `${c.yellow}🔗 ${index}: ${ctx.action}${c.reset}`;

      ctxLines.push(`${prefix}${head} ${label}`);

      const childPrefix = prefix + (isLast ? "   " : `${branch}  `);

      // Messages
      for (const msg of ctx.messages) {
        ctxLines.push(
          `${childPrefix}${tee} ${c.cyan}💬 ${msg}${c.reset}`
        );
      }

      // Attachments
      ctx.attachements.forEach((att, i) => {
        const marker = tee;
        const base = `${childPrefix}${marker} ${c.yellow}📎 ${att.name}:${c.reset}`;

        if (att.value.includes("\n")) {
          ctxLines.push(base);
          ctxLines.push(
            ...formatStack(att.value, childPrefix + `${branch}  `)
          );
        } else {
          ctxLines.push(
            `${base} ${c.gray}${att.value}${c.reset}`
          );
        }
      });

      return ctxLines;
    };

    lines.push(`${c.red}🌪️ TextualError${c.reset}`);

    lines.push(
      ...formatContext(
        this.context,
        "",
        this.earlierContexts.length === 0
      )
    );

    if (this.earlierContexts.length > 0) {
      lines.push("");
      lines.push(`${c.yellow}🌊 Caused by:${c.reset}`);

      const reversed = this.earlierContexts.slice().reverse();

      reversed.forEach((ctx, i) => {
        const isLastCtx = i === reversed.length - 1;
        lines.push(...formatContext(ctx, "", isLastCtx, i));
        if (!isLastCtx) lines.push("");
      });
    }

    return lines.join("\n");
  }
}

// const err = TextualError
//   .create("failed to parse config")
//   .addMessage("invalid JSON format")
//   .addStringAttachment("file", "/etc/app/config.json");

// err
//   .changeContext("reading config file")
//   .addBooleanAttachment("retry", false);

// err
//   .changeContext("opening file")
//   .addErrorAttachment("io_error", new Error("file not found"));

// console.log(err.print({ color: true }));