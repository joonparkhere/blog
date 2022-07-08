package hellojpa;

import javax.persistence.*;

@Entity
@Table(name = "TEST_PRODUCT")
public class Product {

    @Id
    @GeneratedValue
    @Column(name = "PRODUCT_ID")
    private Long id;

    private String name;

    private int price;

    private int stockAmount;

}
