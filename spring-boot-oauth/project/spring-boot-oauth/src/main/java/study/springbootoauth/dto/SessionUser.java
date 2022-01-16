package study.springbootoauth.dto;

import lombok.Getter;
import study.springbootoauth.domain.User;

@Getter
public class SessionUser {

    private final String name;
    private final String email;
    private final String picture;

    public SessionUser(User user) {
        this.name = user.getName();
        this.email = user.getEmail();
        this.picture = user.getPicture();
    }

}
