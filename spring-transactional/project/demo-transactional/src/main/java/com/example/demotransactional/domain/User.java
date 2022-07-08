package com.example.demotransactional.domain;

import lombok.AccessLevel;
import lombok.Getter;
import lombok.NoArgsConstructor;

import javax.persistence.*;
import java.util.ArrayList;
import java.util.List;

@Entity
@Getter
@NoArgsConstructor(access = AccessLevel.PROTECTED)
public class User {

    @Id @GeneratedValue
    @Column(name = "user_id")
    private Long id;

    private String nickname;

    private int age;

    @OneToMany(mappedBy = "user")
    private List<Orders> ordersList = new ArrayList<>();

    public User(String nickname, int age) {
        this.nickname = nickname;
        this.age = age;
    }

    public void modifyNickname(String nickname) {
        this.nickname = nickname;
    }

}
