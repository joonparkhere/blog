package com.example.demotransactional.service;

import com.example.demotransactional.domain.Orders;
import com.example.demotransactional.domain.User;
import com.example.demotransactional.repository.OrderRepository;
import com.example.demotransactional.repository.UserRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
@RequiredArgsConstructor
public class UserService {

    private final UserRepository userRepository;
    private final OrderRepository orderRepository;

    @Transactional(readOnly = true)
    public User loadById(Long id) {
        return userRepository.findById(id).orElseThrow(() -> {
            throw new RuntimeException("User is not exist");
        });
    }

    @Transactional(readOnly = true)
    public User loadWithOrdersById(Long id) {
        return userRepository.findUserWithOrdersListById(id).orElseThrow(() -> {
            throw new RuntimeException("User is not exits");
        });
    }

    @Transactional
    public User save(String nickname, int age) {
        User newUser = new User(nickname, age);

        return userRepository.save(newUser);
    }

    @Transactional
    public User changeNickname(Long userId, String newNickname) {
        User findUser = loadById(userId);
        findUser.modifyNickname(newNickname);

        return findUser;
    }

    @Transactional
    public User orderAsLazyLoading(Long userId, String item) {
        User findUser = loadById(userId);

        Orders orders = new Orders(item);
        orders.addUser(findUser);
        orderRepository.save(orders);

        return findUser;
    }

    @Transactional
    public User orderAsFetchJoin(Long userId, String item) {
        User findUser = loadWithOrdersById(userId);

        Orders orders = new Orders(item);
        orders.addUser(findUser);
        orderRepository.save(orders);

        return findUser;
    }

}
