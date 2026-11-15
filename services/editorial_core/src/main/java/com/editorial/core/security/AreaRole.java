package com.editorial.core.security;

import com.editorial.core.entity.AreaCode;

public final class AreaRole {

    public static final String PREFIX = "ROLE_";

    private AreaRole() {
    }

    public static String roleName(AreaCode area) {
        return PREFIX + area.name();
    }
}