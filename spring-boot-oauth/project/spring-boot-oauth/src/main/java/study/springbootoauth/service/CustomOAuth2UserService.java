package study.springbootoauth.service;

import lombok.RequiredArgsConstructor;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.context.SecurityContext;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.oauth2.client.userinfo.DefaultOAuth2UserService;
import org.springframework.security.oauth2.client.userinfo.OAuth2UserRequest;
import org.springframework.security.oauth2.client.userinfo.OAuth2UserService;
import org.springframework.security.oauth2.core.OAuth2AuthenticationException;
import org.springframework.security.oauth2.core.user.DefaultOAuth2User;
import org.springframework.security.oauth2.core.user.OAuth2User;
import org.springframework.security.oauth2.core.user.OAuth2UserAuthority;
import org.springframework.stereotype.Service;
import study.springbootoauth.domain.Role;
import study.springbootoauth.dto.OAuthAttributes;
import study.springbootoauth.dto.SessionUser;
import study.springbootoauth.domain.User;
import study.springbootoauth.repository.UserRepository;

import javax.servlet.http.HttpSession;
import java.util.Collection;
import java.util.Collections;
import java.util.Map;
import java.util.Optional;

@Service
@RequiredArgsConstructor
public class CustomOAuth2UserService implements OAuth2UserService<OAuth2UserRequest, OAuth2User> {

    private final UserRepository userRepository;
    private final HttpSession httpSession;

    @Override
    public OAuth2User loadUser(OAuth2UserRequest oAuth2UserRequest) throws OAuth2AuthenticationException {
        DefaultOAuth2UserService delegate = new DefaultOAuth2UserService();
        OAuth2User oAuth2User = delegate.loadUser(oAuth2UserRequest);

        Collection<? extends GrantedAuthority> authorities = oAuth2User.getAuthorities();
        Map<String, Object> attributes = oAuth2User.getAttributes();

        String registrationId = oAuth2UserRequest.getClientRegistration()
                .getRegistrationId();
        String userNameAttributeName = oAuth2UserRequest.getClientRegistration()
                .getProviderDetails().getUserInfoEndpoint().getUserNameAttributeName();

        OAuthAttributes oAuthAttributes = OAuthAttributes.of(registrationId, userNameAttributeName, attributes);
        User user = saveOrUpdate(oAuthAttributes);
        httpSession.setAttribute("user", new SessionUser(user));

        String roleKey = null;
        if (authorities.contains(new OAuth2UserAuthority(attributes))) {
            roleKey = Role.USER.getKey();
        }
        else {
            roleKey = Role.GUEST.getKey();
        }

        return new DefaultOAuth2User(
                Collections.singleton(new SimpleGrantedAuthority(roleKey)),
                oAuthAttributes.getAttributes(),
                oAuthAttributes.getNameAttributeKey()
        );
    }

    private User saveOrUpdate(OAuthAttributes attributes) {
        Optional<User> optionalUser = userRepository.findByEmail(attributes.getEmail())
                .map(entity -> entity.update(attributes.getName(), attributes.getPicture()));

        User user = userRepository.findByEmail(attributes.getEmail())
                .map(entity -> entity.update(attributes.getName(), attributes.getPicture()))
                .orElse(attributes.toEntity());

        return user;

//        return userRepository.save(user);
    }
}
