package com.editorial.core.service;

import com.editorial.core.entity.Employee;
import com.editorial.core.entity.JobOpening;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.exception.ErrorResponses;
import com.editorial.core.model.request.CreateEmployeeRequest;
import com.editorial.core.model.response.EmployeeResponse;
import com.editorial.core.repository.EmployeeRepository;
import com.editorial.core.model.request.CreateJobOpeningRequest;
import com.editorial.core.model.response.JobOpeningResponse;
import com.editorial.core.repository.JobOpeningRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.Instant;
import java.time.LocalDate;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class HrService {

    private final JobOpeningRepository jobOpeningRepository;
    private final EmployeeRepository employeeRepository;

    @Transactional
    public EmployeeResponse createEmployee(CreateEmployeeRequest request) throws ControllerException {
        String code = request.getEmployeeCode().trim();
        if (employeeRepository.existsByEmployeeCode(code)) {
            throw new ControllerException(ErrorResponses.CONFLICT);
        }
        Employee employee = new Employee();
        employee.setId(UUID.randomUUID());
        employee.setEmployeeCode(code);
        employee.setFullName(request.getFullName().trim());
        employee.setEmail(request.getEmail().trim());
        employee.setDepartment(request.getDepartment());
        employee.setHiredAt(request.getHiredAt() != null ? request.getHiredAt() : LocalDate.now());
        employee.setActive(true);
        employee = employeeRepository.save(employee);
        return new EmployeeResponse(
                employee.getId(),
                employee.getEmployeeCode(),
                employee.getFullName(),
                employee.getDepartment().name(),
                employee.isActive()
        );
    }

    @Transactional
    public JobOpeningResponse createJobOpening(CreateJobOpeningRequest request) {
        JobOpening opening = new JobOpening();
        opening.setId(UUID.randomUUID());
        opening.setTitle(request.getTitle().trim());
        opening.setDescription(request.getDescription());
        opening.setArea(request.getArea());
        opening.setOpen(true);
        opening.setCreatedAt(Instant.now());
        opening = jobOpeningRepository.save(opening);
        return new JobOpeningResponse(
                opening.getId(),
                opening.getTitle(),
                opening.getArea().name(),
                opening.isOpen()
        );
    }
}