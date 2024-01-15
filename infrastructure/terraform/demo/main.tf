terraform {
    required_providers {
        equinix = {
            source = "equinix/equinix"
        }
    }
}

provider equinix {
    auth_token = replace(file("./.secrets/equinix_auth_token"), "\n", "")
}

data local_file project_id {
    filename = "./.secrets/equinix_project_id"
}

resource equinix_metal_device far_memory_app {
    hostname = "far-memory-app"
    plan = "m3.small.x86" # has 25gbps NIC
    metro = "fr"
    operating_system = "ubuntu_22_04"
    billing_cycle = "hourly"
    project_id = replace(data.local_file.project_id.content, "\n", "")
}

resource equinix_metal_device far_memory_storage {
    hostname = "far-memory-storage"
    plan = "m3.small.x86" # has 25gbps NIC
    metro = "fr"
    operating_system = "ubuntu_22_04"
    billing_cycle = "hourly"
    project_id = replace(data.local_file.project_id.content, "\n", "")
}
