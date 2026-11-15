package com.editorial.core.repository;

import com.editorial.core.entity.AppUser;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.Optional;
import java.util.UUID;

public interface AppUserRepository extends JpaRepository<AppUser, UUID> {

    Optional<AppUser> findByLdapUidIgnoreCase(String ldapUid);
}