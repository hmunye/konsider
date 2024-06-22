import { Post } from "@/types/types";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "./Card";

export default function PostCard({ post }: { post: Post }) {
  return (
    <Card className="w-96 m-10">
      <CardHeader>
        <CardTitle>Title: {post.title}</CardTitle>
        <CardDescription>Content: {post.content}</CardDescription>
      </CardHeader>
      <CardContent>Published: {post.published ? "Yes" : "No"}</CardContent>
      <CardFooter className="flex justify-between">{post.id}</CardFooter>
    </Card>
  );
}
