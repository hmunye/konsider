// TODO: Fix this middleware authorization check

import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { checkAuth } from "./lib/api";

export async function middleware(request: NextRequest) {
  const response = await checkAuth();

  if (response.error) {
    return NextResponse.redirect(new URL("/", request.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: [
    // Match all request paths except:
    // - _next/static (static files)
    // - _next/image (image optimization files)
    // - favicon.ico, sitemap.xml, robots.txt (metadata files)
    // - /
    "/((?!_next/static|_next/image|favicon.ico|sitemap.xml|robots.txt|$).*)",
  ],
};
