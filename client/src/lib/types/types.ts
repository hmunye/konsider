export type Message =
  | { success: string }
  | { error: string }
  | { message: string };

export type Metadata = {
  current_page: number;
  per_page: number;
  first_page: number;
  last_page: number;
  total_records: number;
};

export type User = {
  id: string;
  name: string;
  email: string;
  role: "ADMIN" | "REVIEWER";
  created_at: string;
};

export type UserResponse = {
  metadata: Metadata;
  users: Array<{ user: User }>;
};

export type Software = {
  id: string;
  software_name: string;
  software_version: string;
  developer_name: string;
  description: string;
  created_at: string;
};

export type SoftwareResponse = {
  metadata: Metadata;
  software: Array<{ software: Software }>;
};

export type Requester = {
  id: string;
  name: string;
  email: string;
  department: string;
  created_at: string;
};

export type RequesterResponse = {
  metadata: Metadata;
  requesters: Array<{ request: Request }>;
};

export type SoftwareRequest = {
  id: string;
  td_request_id: string;
  software: Software;
  requester: Requester;
  created_at: string;
};

export type SoftwareRequestResponse = {
  metadata: Metadata;
  software_requests: Array<{ software_request: SoftwareRequest }>;
};

export type SoftwareReview = {
  id: string;
  software_request: SoftwareRequest;
  reviewer: User;
  is_supported: "TRUE" | "FALSE" | "NOT_SURE";
  is_current_version: "TRUE" | "FALSE" | "NOT_SURE";
  is_reputation_good: "TRUE" | "FALSE" | "NOT_SURE";
  is_installation_from_developer: "TRUE" | "FALSE" | "NOT_SURE";
  is_local_admin_required: "TRUE" | "FALSE" | "NOT_SURE";
  is_connected_to_brockport_cloud: "TRUE" | "FALSE" | "NOT_SURE";
  is_connected_to_cloud_services_or_client: "TRUE" | "FALSE" | "NOT_SURE";
  is_security_or_optimization_software: "TRUE" | "FALSE" | "NOT_SURE";
  is_supported_by_current_os: "TRUE" | "FALSE" | "NOT_SURE";
  exported: boolean;
  review_notes: string;
  created_at: string;
};

export type SoftwareReviewResponse = {
  metadata: Metadata;
  software_reviews: Array<{ software_review: SoftwareReview }>;
};
