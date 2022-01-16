package study.springbootoauth.controller;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.authentication.AnonymousAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.context.SecurityContext;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.oauth2.client.OAuth2AuthorizedClient;
import org.springframework.security.oauth2.client.OAuth2AuthorizedClientService;
import org.springframework.security.oauth2.client.authentication.OAuth2AuthenticationToken;
import org.springframework.security.oauth2.core.DefaultOAuth2AuthenticatedPrincipal;
import org.springframework.security.oauth2.core.OAuth2AccessToken;
import org.springframework.security.oauth2.core.OAuth2AuthenticatedPrincipal;
import org.springframework.security.oauth2.core.user.DefaultOAuth2User;
import org.springframework.security.oauth2.core.user.OAuth2User;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import study.springbootoauth.domain.Role;
import study.springbootoauth.domain.User;
import study.springbootoauth.dto.OAuthAttributes;
import study.springbootoauth.service.CustomOAuth2UserService;
import study.springbootoauth.service.UserService;

import java.security.Principal;
import java.util.Collection;
import java.util.Map;

@Controller
@RequestMapping("/signUp")
public class SignUpController {

    @Autowired
    CustomOAuth2UserService customOAuth2UserService;

    @Autowired
    UserService userService;

    //
    @Autowired
    OAuth2AuthorizedClientService oAuth2AuthorizedClientService;
    //

    @GetMapping
    public String signUpForm(Model model, Authentication authentication) {
        OAuth2AuthenticationToken oAuth2AuthenticationToken = (OAuth2AuthenticationToken) authentication;
        String authorizedClientRegistrationId = oAuth2AuthenticationToken.getAuthorizedClientRegistrationId();
        OAuth2User oAuth2User = oAuth2AuthenticationToken.getPrincipal();
        String nameAttributeKey = oAuth2User.getName();
        Map<String, Object> attributes = oAuth2User.getAttributes();

        OAuthAttributes oAuthAttributes = OAuthAttributes.of(authorizedClientRegistrationId, nameAttributeKey, attributes);
        User user = oAuthAttributes.toEntity();

//        SecurityContextHolder.clearContext();

        //
//        OAuth2AuthorizedClient oAuth2AuthorizedClient = oAuth2AuthorizedClientService.loadAuthorizedClient("google", authentication.getName());
//        OAuth2AccessToken oAuth2AccessToken = oAuth2AuthorizedClient.getAccessToken();
        //

        model.addAttribute("user", user);
        return "signUp";
    }

    @PostMapping
    public String processSignUp(@ModelAttribute User user) {
        SecurityContext context = SecurityContextHolder.getContext();
        user.setRole(Role.USER);
        userService.createUser(user);
        return "redirect:/dashboard";
    }

}
