package study.springbootoauth.repository;

import org.springframework.data.jpa.repository.JpaRepository;
import study.springbootoauth.domain.User;

import java.util.Optional;

public interface UserRepository extends JpaRepository<User, Long> {
    Optional<User> findByEmail(String email);
    User findUserByEmail(String email);
}
