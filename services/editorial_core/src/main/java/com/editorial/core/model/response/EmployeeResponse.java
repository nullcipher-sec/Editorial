package com.editorial.core.model.response;

import java.util.UUID;

public record EmployeeResponse(UUID id, String employeeCode, String fullName, String department, boolean active) {
}
