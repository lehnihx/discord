export const app = {
  name: "iLenix",
  owner: "Lenix",
  contact: "contact@lenix.dev",
  effectiveDate: "June 27, 2026",
};

export type LegalPage = {
  slug: "terms" | "privacy";
  title: string;
  description: string;
  updated: string;
  sections: {
    title: string;
    body: string[];
    list?: string[];
  }[];
};

export const pages: Record<LegalPage["slug"], LegalPage> = {
  terms: {
    slug: "terms",
    title: "Terms of Service",
    description: `Rules for using ${app.name}.`,
    updated: app.effectiveDate,
    sections: [
      {
        title: "Acceptance",
        body: [
          `By inviting, accessing, or using ${app.name}, you agree to these Terms of Service and to Discord's Terms of Service and Community Guidelines.`,
        ],
      },
      {
        title: "Permitted Use",
        body: [
          `${app.name} is provided to help Discord communities use bot features, commands, automation, and server utilities. You are responsible for how you configure and use the application in your server.`,
        ],
      },
      {
        title: "Prohibited Use",
        body: ["You may not use the application to:"],
        list: [
          "Violate Discord's Terms of Service, Developer Terms, or Community Guidelines.",
          "Send spam, harassment, harmful content, or illegal material.",
          "Abuse, disrupt, reverse engineer, or attempt to gain unauthorized access to the application or its infrastructure.",
          "Collect, expose, or misuse personal information from Discord users.",
        ],
      },
      {
        title: "Access and Enforcement",
        body: [
          "We may restrict, suspend, or remove access to the application when needed to prevent abuse, protect service reliability, comply with law, or enforce these terms.",
        ],
      },
      {
        title: "Availability",
        body: [
          "The application is provided as-is and may change, break, or become unavailable at any time. We do not provide warranties about uptime, accuracy, or fitness for a particular purpose.",
        ],
      },
      {
        title: "Contact",
        body: [`Questions about these terms can be sent to: ${app.contact}.`],
      },
    ],
  },
  privacy: {
    slug: "privacy",
    title: "Privacy Policy",
    description: `How ${app.name} handles Discord data.`,
    updated: app.effectiveDate,
    sections: [
      {
        title: "Data We Process",
        body: [
          `${app.name} may process Discord user IDs, server IDs, channel IDs, role IDs, message or command content provided to the bot, configuration values, and usage logs needed to operate and secure the application.`,
        ],
      },
      {
        title: "How We Use Data",
        body: ["We use data only for application-related purposes, including:"],
        list: [
          "Responding to commands and providing configured bot features.",
          "Saving server preferences and application settings.",
          "Preventing abuse, debugging errors, and improving reliability.",
          "Complying with legal obligations or valid platform requirements.",
        ],
      },
      {
        title: "Sharing",
        body: [
          "We do not sell personal data. Data may be processed by hosting, database, logging, or infrastructure providers only as needed to operate the application, or disclosed when required by law.",
        ],
      },
      {
        title: "Retention",
        body: [
          "We retain stored data only as long as needed for the application to function, for security, or for legal compliance. Temporary logs may be deleted on a rolling basis.",
        ],
      },
      {
        title: "User Choices and Deletion",
        body: [
          "Server administrators can remove the application from a server at any time. Users or server owners may request deletion of stored data by contacting us with enough information to identify the relevant Discord user or server records.",
        ],
      },
      {
        title: "Security",
        body: [
          "We use reasonable technical and organizational measures to protect stored data, but no online service can guarantee absolute security.",
        ],
      },
      {
        title: "Contact",
        body: [`Privacy questions or deletion requests can be sent to: ${app.contact}.`],
      },
    ],
  },
};
