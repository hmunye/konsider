export type Message =
  | { success: string }
  | { error: string }
  | { message: string };

export type LoginResponse = {
  role: "ADMIN" | "REVIEWER";
};

export type Metadata = {
  current_page: number;
  first_page: number;
  last_page: number;
  per_page: number;
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
  description: string;
  developer: string;
  id: string;
  name: string;
  software_version: string;
  created_at: string;
};

export type SoftwareResponse = {
  metadata: Metadata;
  software: Array<{ software: Software }>;
};

export type Request = {
  department: string;
  email: string;
  id: string;
  name: string;
  td_request_id: number;
  created_at: string;
};

export type RequestResponse = {
  metadata: Metadata;
  software_requests: Array<{ request: Request }>;
};

export type Review = {
  exported: boolean;
  id: string;
  is_connected_to_brockport_cloud: "TRUE" | "FALSE" | "NOT_SURE";
  is_connected_to_cloud_services_or_client: "TRUE" | "FALSE" | "NOT_SURE";
  is_current_version: "TRUE" | "FALSE" | "NOT_SURE";
  is_installation_from_developer: "TRUE" | "FALSE" | "NOT_SURE";
  is_local_admin_required: "TRUE" | "FALSE" | "NOT_SURE";
  is_reputation_good: "TRUE" | "FALSE" | "NOT_SURE";
  is_security_or_optimization_software: "TRUE" | "FALSE" | "NOT_SURE";
  is_supported: "TRUE" | "FALSE" | "NOT_SURE";
  is_supported_by_current_os: "TRUE" | "FALSE" | "NOT_SURE";
  notes: string;
  reviewer_name: string;
  software_name: string;
  td_request_id: number;
  created_at: string;
};

export type ReviewResponse = {
  metadata: Metadata;
  software_reviews: Array<{ review: Review }>;
};
