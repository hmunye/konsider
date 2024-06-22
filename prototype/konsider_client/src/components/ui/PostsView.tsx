import { useFetch } from "@/hooks/useFetch";
import { Post } from "@/types/types";
import { useQuery } from "@tanstack/react-query";
import PostCard from "./PostCard";

const apiUrl = "http://127.0.0.1:8000/posts";

export default function PostsView() {
  const apiQuery = useQuery<Post[]>({
    queryKey: ["id"],
    queryFn: async () => {
      const responseData = await useFetch({ url: apiUrl, method: "GET" });
      return responseData.data;
    },
  });

  const { data: posts, status } = apiQuery;

  if (status == "pending") {
    return (
      <div className="flex flex-1 items-center justify-center rounded-lg flex-col gap-4 p-4 lg:gap-6 lg:p-6">
        <div className="flex flex-col items-center gap- text-center">
          <h3 className="text-2xl font-bold tracking-tight mb-5">Posts</h3>
          <p className="text-sm text-muted-foreground">Loading...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-1 items-center justify-center rounded-lg flex-col gap-4 p-4 lg:gap-6 lg:p-6">
      <div className="flex flex-col items-center gap- text-center">
        <h3 className="text-2xl font-bold tracking-tight mb-5">Posts</h3>
        {posts && posts.length > 0 ? (
          <div className="">
            {posts.map((post) => (
              <PostCard post={post} key={post.id} />
            ))}
          </div>
        ) : (
          <p className="text-sm text-muted-foreground">No Posts Available</p>
        )}
      </div>
    </div>
  );
}
