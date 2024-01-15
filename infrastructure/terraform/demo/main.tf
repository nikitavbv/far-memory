terraform {
    required_providers {
        equinix = {
            source = "equinix/equinix"
        }
    }
}

provider equinix {
    auth_token = file("./secrets/equinix_auth_token")
}
