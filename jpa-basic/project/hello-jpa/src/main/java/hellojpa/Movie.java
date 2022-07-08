package hellojpa;

import javax.persistence.DiscriminatorValue;
import javax.persistence.Entity;
import javax.persistence.Table;

@Entity
@Table(name = "TEST_MOVIE")
@DiscriminatorValue(value = "M")
public class Movie extends Item {
    private String director;
    private String actor;
}
