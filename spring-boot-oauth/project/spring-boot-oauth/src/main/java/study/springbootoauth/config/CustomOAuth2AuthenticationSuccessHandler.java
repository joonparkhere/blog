package study.springbootoauth.config;

import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.security.authentication.AnonymousAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContext;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.oauth2.core.user.DefaultOAuth2User;
import org.springframework.security.web.WebAttributes;
import org.springframework.security.web.authentication.AnonymousAuthenticationFilter;
import org.springframework.security.web.authentication.SavedRequestAwareAuthenticationSuccessHandler;
import study.springbootoauth.domain.User;
import study.springbootoauth.service.UserService;

import javax.servlet.FilterChain;
import javax.servlet.ServletException;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.servlet.http.HttpSession;
import java.io.IOException;

@RequiredArgsConstructor
public class CustomOAuth2AuthenticationSuccessHandler extends SavedRequestAwareAuthenticationSuccessHandler {

    private final UserService userService;

    @Override
    public void onAuthenticationSuccess(HttpServletRequest request, HttpServletResponse response, FilterChain chain, Authentication authentication) throws IOException, ServletException {

    }

    @Override
    public void onAuthenticationSuccess(HttpServletRequest request, HttpServletResponse response, Authentication authentication) throws IOException, ServletException {
        Object principal = authentication.getPrincipal();
        if (principal instanceof DefaultOAuth2User) {
            DefaultOAuth2User defaultOAuth2User = (DefaultOAuth2User) principal;
            String email = (String) defaultOAuth2User.getAttributes().get("email");

            User user = userService.findUserByEmail(email);
            if (user != null) {
                super.onAuthenticationSuccess(request, response, authentication);
                // TODO 기존 인증 Update
            }
            else {
                HttpSession session = request.getSession(false);
                SecurityContext securityContext = SecurityContextHolder.getContext();
//                AnonymousAuthenticationToken anonymousAuthenticationToken = new AnonymousAuthenticationToken(key.hashCode());
//                securityContext.setAuthentication();
//                SecurityContextHolder.getContext().setAuthentication(createAuthentication((HttpServletRequest) request));
//                SecurityContextHolder.getContext().setAuthentication(null);
                clearAuthenticationAttributes(request);
                String targetUrl = "/signUp";
                getRedirectStrategy().sendRedirect(request, response, targetUrl);
            }
        }
    }

}
