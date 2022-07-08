package com.example.demotransactional.domain;

import lombok.AccessLevel;
import lombok.Getter;
import lombok.NoArgsConstructor;

import javax.persistence.*;

@Entity
@Getter
@NoArgsConstructor(access = AccessLevel.PROTECTED)
public class Orders {

    @Id @GeneratedValue
    @Column(name = "orders_id")
    private Long id;

    private String item;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "user_id")
    private User user;

    public Orders(String item) {
        this.item = item;
    }

    public void addUser(User user) {
        this.user = user;
        user.getOrdersList().add(this);
    }

}
