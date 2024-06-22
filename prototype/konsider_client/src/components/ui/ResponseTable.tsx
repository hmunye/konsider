import { UseQueryResult } from "@tanstack/react-query";
import Link from "next/link";

export default function ResponseTable({
  query,
  url,
}: {
  query: UseQueryResult<any, Error>;
  url: string;
}) {
  return (
    <table className="border-collapse border-2">
      <thead>
        <tr>
          <th className="border px-4 py-2 text-center">API Endpoint</th>
          <th className="border px-4 py-2 text-center">API Response</th>
          <th className="border px-4 py-2 text-center">Response Status</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td className="border px-4 py-2 text-center">
            <Link href={url} target="_blank" className="hover:underline">
              {url}
            </Link>
          </td>
          <td className="border px-4 py-2 text-center">
            {query.isLoading
              ? "Loading..."
              : query.isError
              ? `${query.error.message}`
              : query.data.message}
          </td>
          <td className="border px-4 py-2 text-center">
            {query.isLoading ? (
              "Loading..."
            ) : query.status == "success" ? (
              <span className="text-primary">{query.status.toUpperCase()}</span>
            ) : (
              <span className="text-destructive">
                {query.status.toUpperCase()}
              </span>
            )}
          </td>
        </tr>
      </tbody>
    </table>
  );
}
