package hellojpa;

import javax.persistence.DiscriminatorValue;
import javax.persistence.Entity;
import javax.persistence.Table;

@Entity
@Table(name = "TEST_BOOK")
@DiscriminatorValue(value = "B")
public class Book extends Item {
    private String author;
    private String isbn;
}
