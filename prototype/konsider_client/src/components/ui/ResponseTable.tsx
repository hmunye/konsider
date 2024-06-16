import { UseQueryResult } from "@tanstack/react-query";

export default function ResponseTable({
  query,
  url,
}: {
  query: UseQueryResult<any, Error>;
  url: string;
}) {
  return (
    <table className="border-collapse border-2 border-foreground">
      <thead>
        <tr>
          <th className="border border-gray-300 px-4 py-2 text-center">
            API Endpoint
          </th>
          <th className="border border-gray-300 px-4 py-2 text-center">
            API Response
          </th>
          <th className="border border-gray-300 px-4 py-2 text-center">
            Response Status
          </th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td className="border border-gray-300 px-4 py-2 text-center">
            {url}
          </td>
          <td className="border border-gray-300 px-4 py-2 text-center">
            {query.isLoading
              ? "Loading..."
              : query.isError
              ? `Error: ${query.error.message}`
              : query.data.message}
          </td>
          <td className="border border-gray-300 px-4 py-2 text-center">
            {query.isLoading ? (
              "Loading..."
            ) : query.status == "success" ? (
              <span className="text-green-300">{query.status}</span>
            ) : (
              <span className="text-red-500">{query.status}</span>
            )}
          </td>
        </tr>
      </tbody>
    </table>
  );
}
