package hellojpa;

import javax.persistence.DiscriminatorValue;
import javax.persistence.Entity;
import javax.persistence.Table;

@Entity
@Table(name = "TEST_ALBUM")
@DiscriminatorValue(value = "A")
public class Album extends Item {
    private String artist;
}
