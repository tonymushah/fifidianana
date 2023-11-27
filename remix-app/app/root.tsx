import { ChakraProvider, Box } from "@chakra-ui/react";
import type { LinksFunction, MetaFunction } from "@remix-run/node";
import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import { cssBundleHref } from "@remix-run/css-bundle";

import '@fontsource/poppins/index.css';
import theme from "./theme";
import RootErrorBoundary from "./components/ErrorBoundary";

export const meta: MetaFunction = () => {
  return (
    [
      { charset: "utf-8" },
      { viewport: "width=device-width,initial-scale=1" }
    ]
  )
};

export const links: LinksFunction = () => {
  return [
    ...(cssBundleHref
      ? [{ rel: "stylesheet", href: cssBundleHref }]
      : []),
    // ...
  ];
};

function Document({
  children,
  title = "App title",
}: {
  children: React.ReactNode;
  title?: string;
}) {
  return (
    <html lang="en">
      <head>
        <Meta />
        <title>{title}</title>
        <Links />
      </head>
      <body>
        {children}
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}

export default function App() {
  // throw new Error("💣💥 Booooom");

  return (
    <Document>
      <ChakraProvider theme={theme}>
        <Outlet />
      </ChakraProvider>
    </Document>
  );
}

// How ChakraProvider should be used on ErrorBoundary
export function ErrorBoundary() {
  return (
    <Document title="Error!">
      <ChakraProvider>
        <Box>
          <RootErrorBoundary />
        </Box>
      </ChakraProvider>
    </Document>
  );
}
