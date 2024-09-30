import { API_URL, fetchData } from "@/src/lib/api";
import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";

export async function middleware(request: NextRequest) {
  const req_cookie = request.cookies.get("id");
  const cookie = req_cookie ? `${req_cookie.name}=${req_cookie.value}` : "";

  try {
    const response = await fetchData({
      url: `${API_URL}/v1/auth/check`,
      cookie: cookie,
      method: "GET",
    });

    if (response.error) {
      return NextResponse.redirect(new URL("/", request.url));
    }

    return NextResponse.next();
  } catch {
    throw new Error("An error occurred during authentication check");
  }
}

export const config = {
  matcher: [
    // Match all request paths except:
    // - _next/static (static files)
    // - _next/image (image optimization files)
    // - favicon.ico, sitemap.xml, robots.txt (metadata files)
    // - / (home page)
    "/((?!_next/static|_next/image|favicon.ico|sitemap.xml|robots.txt|$).*)",
  ],
};
