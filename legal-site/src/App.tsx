import type React from "react";
import { ArrowUpRight, FileText, ShieldCheck } from "lucide-react";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
import { app, pages, type LegalPage } from "./legal";
import { buttonVariants } from "./components/ui/button";

function pageFromPath(): LegalPage | null {
  const path = window.location.pathname.toLowerCase();
  if (path.includes("/privacy")) return pages.privacy;
  if (path.includes("/terms")) return pages.terms;
  return null;
}

function Nav() {
  const nestedRoute = pageFromPath() !== null;
  const routePrefix = nestedRoute ? "../" : "./";

  return (
    <header className="border-b border-border bg-card/85 backdrop-blur">
      <div className="mx-auto flex max-w-5xl items-center justify-between px-6 py-4">
        <a
          href={nestedRoute ? "../" : "./"}
          className="text-sm font-semibold text-foreground"
        >
          {app.name}
        </a>
        <nav className="flex items-center gap-2">
          <a
            className={buttonVariants({ variant: "ghost", size: "sm" })}
            href={`${routePrefix}terms/`}
          >
            Terms
          </a>
          <a
            className={buttonVariants({ variant: "ghost", size: "sm" })}
            href={`${routePrefix}privacy/`}
          >
            Privacy
          </a>
        </nav>
      </div>
    </header>
  );
}

function Home() {
  return (
    <main className="mx-auto grid min-h-[calc(100vh-73px)] max-w-5xl content-center gap-8 px-6 py-12">
      <section className="max-w-3xl">
        <p className="text-sm font-medium uppercase tracking-wide text-muted-foreground">
          Legal
        </p>
        <h1 className="mt-4 text-4xl font-bold tracking-normal text-foreground sm:text-5xl">
          Terms and privacy for {app.name}
        </h1>
        <p className="mt-5 max-w-2xl text-lg leading-8 text-muted-foreground">
          Public policy pages for Discord verification and user transparency.
        </p>
      </section>
      <section className="grid gap-4 sm:grid-cols-2">
        <LegalLink
          href="terms/"
          icon={<FileText className="h-5 w-5" />}
          title="Terms of Service"
          description="Rules for using the application."
        />
        <LegalLink
          href="privacy/"
          icon={<ShieldCheck className="h-5 w-5" />}
          title="Privacy Policy"
          description="How Discord data is handled."
        />
      </section>
    </main>
  );
}

function LegalLink({
  href,
  icon,
  title,
  description,
}: {
  href: string;
  icon: React.ReactNode;
  title: string;
  description: string;
}) {
  return (
    <a
      href={href}
      className="block focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
    >
      <Card className="h-full transition-colors hover:border-primary/50">
        <CardHeader>
          <div className="flex items-center justify-between gap-4">
            <div className="flex h-10 w-10 items-center justify-center rounded-md bg-secondary text-secondary-foreground">
              {icon}
            </div>
            <ArrowUpRight className="h-4 w-4 text-muted-foreground" />
          </div>
          <CardTitle>{title}</CardTitle>
          <CardDescription>{description}</CardDescription>
        </CardHeader>
      </Card>
    </a>
  );
}

function LegalDocument({ page }: { page: LegalPage }) {
  return (
    <main className="mx-auto max-w-4xl px-6 py-12">
      <div className="mb-8">
        <p className="text-sm font-medium uppercase tracking-wide text-muted-foreground">
          Updated {page.updated}
        </p>
        <h1 className="mt-3 text-4xl font-bold tracking-normal text-foreground">
          {page.title}
        </h1>
        <p className="mt-4 text-lg text-muted-foreground">{page.description}</p>
      </div>
      <Card>
        <CardContent className="legal-copy pt-6">
          {page.sections.map((section) => (
            <section key={section.title}>
              <h2>{section.title}</h2>
              {section.body.map((paragraph) => (
                <p key={paragraph}>{paragraph}</p>
              ))}
              {section.list ? (
                <ul>
                  {section.list.map((item) => (
                    <li key={item}>{item}</li>
                  ))}
                </ul>
              ) : null}
            </section>
          ))}
        </CardContent>
      </Card>
    </main>
  );
}

export default function App() {
  const page = pageFromPath();

  return (
    <>
      <Nav />
      {page ? <LegalDocument page={page} /> : <Home />}
    </>
  );
}
