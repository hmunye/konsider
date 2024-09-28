import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";
import { CheckAuth } from "./lib/api";

export async function middleware(request: NextRequest) {
  const req_cookie = request.cookies.get("id");
  const cookie = req_cookie ? `${req_cookie.name}=${req_cookie.value}` : "";

  const response = await CheckAuth(cookie);

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
