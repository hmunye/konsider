-- Current factors:
--  is_supported (Is the software still supported by the developer?)
--  is_current_version (Is the latest version of the software being requested?)
--  is_reputation_good (Does the developer have a good reputation?)
--  is_installation_from_developer (Is the installation package from the developer/vendor?)
--  is_local_admin_required (Is a local administrator required for daily use?)
--  is_connected_to_brockport_cloud (Does the software need to connect to Brockport cloud?)
--  is_connected_to_cloud_services_or_client (Does the software need to connect to other cloud services or is a client for a cloud service?)
--  is_security_or_optimization_software (Is the software for security or system optimization?)
--  is_supported_by_current_os (Is the software supported by current OS used by devices on campus?)

CREATE TABLE review_factor (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- Factor name, e.g., 'is_supported' (It is UNIQUE so an INDEX is created automatically)
    description VARCHAR(255) NOT NULL, -- Description providing context for each review factor
    created_at TIMESTAMPTZ DEFAULT NOW()
);
