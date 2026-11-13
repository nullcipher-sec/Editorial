package com.editorial.core.controller.internal;

import com.editorial.core.entity.Employee;
import com.editorial.core.entity.JobOpening;
import com.editorial.core.exception.ControllerException;
import com.editorial.core.model.response.EmployeeResponse;
import com.editorial.core.model.response.JobOpeningResponse;
import com.editorial.core.model.response.PageResponse;
import com.editorial.core.repository.EmployeeRepository;
import com.editorial.core.repository.JobOpeningRepository;
import com.editorial.core.util.Phase1Guard;
import io.swagger.v3.oas.annotations.tags.Tag;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Pageable;
import org.springframework.data.web.PageableDefault;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/internal/hr")
@RequiredArgsConstructor
@Tag(name = "Internal — HR")
public class HrController {

    private final EmployeeRepository employeeRepository;
    private final JobOpeningRepository jobOpeningRepository;

    @GetMapping("/employees")
    public PageResponse<EmployeeResponse> listEmployees(@PageableDefault(size = 20) Pageable pageable) {
        var page = employeeRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toEmployee).getContent());
    }

    @PostMapping("/employees")
    public void createEmployee(@RequestBody Object body) throws ControllerException {
        Phase1Guard.rejectMutation();
    }

    @GetMapping("/job-openings")
    public PageResponse<JobOpeningResponse> listJobOpenings(@PageableDefault(size = 20) Pageable pageable) {
        var page = jobOpeningRepository.findAll(pageable);
        return PageResponse.from(page, page.map(this::toJobOpening).getContent());
    }

    @PostMapping("/job-openings")
    public void createJobOpening(@RequestBody Object body) throws ControllerException {
        Phase1Guard.rejectMutation();
    }

    private EmployeeResponse toEmployee(Employee employee) {
        return new EmployeeResponse(
                employee.getId(),
                employee.getEmployeeCode(),
                employee.getFullName(),
                employee.getDepartment().name(),
                employee.isActive()
        );
    }

    private JobOpeningResponse toJobOpening(JobOpening jobOpening) {
        return new JobOpeningResponse(
                jobOpening.getId(),
                jobOpening.getTitle(),
                jobOpening.getArea().name(),
                jobOpening.isOpen()
        );
    }
}
