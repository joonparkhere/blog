package hellojpa;

import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;
import javax.persistence.Table;

@Entity
@Table(name = "TEST_ADDRESS")
public class AddressEntity {

    @Id
    @GeneratedValue
    private Long id;

    private Address address;

}
