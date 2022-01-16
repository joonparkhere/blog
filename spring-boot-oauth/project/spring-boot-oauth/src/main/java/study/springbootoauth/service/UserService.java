package study.springbootoauth.service;

import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import study.springbootoauth.domain.User;
import study.springbootoauth.repository.UserRepository;

@Service
@RequiredArgsConstructor
public class UserService {

    private final UserRepository userRepository;

    public User findUserByEmail(String email) {
        return userRepository.findUserByEmail(email);
    }

    public User createUser(User user) {
        return userRepository.save(user);
    }

}
