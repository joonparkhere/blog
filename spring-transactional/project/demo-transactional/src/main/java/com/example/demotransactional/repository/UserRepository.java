package com.example.demotransactional.repository;

import com.example.demotransactional.domain.User;
import org.springframework.data.jpa.repository.EntityGraph;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;
import java.util.Optional;

public interface UserRepository extends JpaRepository<User, Long> {

    @EntityGraph(attributePaths = {"ordersList"})
    Optional<User> findUserWithOrdersListById(Long id);

}
