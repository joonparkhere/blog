package study.springbootoauth.config;

import lombok.RequiredArgsConstructor;
import org.springframework.boot.autoconfigure.security.servlet.PathRequest;
import org.springframework.security.access.expression.SecurityExpressionHandler;
import org.springframework.security.access.hierarchicalroles.RoleHierarchyImpl;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.builders.WebSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configuration.WebSecurityConfigurerAdapter;
import org.springframework.security.config.annotation.web.configurers.oauth2.client.OAuth2LoginConfigurer;
import org.springframework.security.web.access.expression.DefaultWebSecurityExpressionHandler;
import org.springframework.security.web.authentication.SimpleUrlAuthenticationSuccessHandler;
import study.springbootoauth.domain.Role;
import study.springbootoauth.service.AuthProvider;
import study.springbootoauth.service.CustomOAuth2UserService;
import study.springbootoauth.service.UserService;

@EnableWebSecurity
@RequiredArgsConstructor
public class SecurityConfig extends WebSecurityConfigurerAdapter {

    private final CustomOAuth2UserService customOAuth2UserService;
    private final UserService userService;
    private final AuthProvider authProvider;

    @Override
    public void configure(WebSecurity web) throws Exception {
        web.ignoring().requestMatchers(PathRequest.toStaticResources().atCommonLocations());
    }

    @Override
    protected void configure(HttpSecurity http) throws Exception {
//        http.csrf().disable()
//                .headers().frameOptions().disable();

        http.authorizeRequests()
                .antMatchers("/", "/login", "/signUp").permitAll()
//                .antMatchers("/api/v1/**").hasRole(Role.USER.name())
                .antMatchers("/dashboard").hasRole(Role.USER.name())
                .anyRequest().authenticated()
                .expressionHandler(expressionHandler());

        http.httpBasic();

        http.oauth2Login()
                .loginPage("/login");

//        http.oauth2Login()
//                .userInfoEndpoint().userService(customOAuth2UserService);

        http.oauth2Login()
                .successHandler(successHandler());

        http.authenticationProvider(authProvider);

        http.logout()
                .logoutUrl("/logout")
                .logoutSuccessUrl("/");
    }

    public SecurityExpressionHandler expressionHandler() {
        RoleHierarchyImpl roleHierarchy = new RoleHierarchyImpl();
        roleHierarchy.setHierarchy("ROLE_ADMIN > ROLE_USER");

        DefaultWebSecurityExpressionHandler handler = new DefaultWebSecurityExpressionHandler();
        handler.setRoleHierarchy(roleHierarchy);

        return handler;
    }

    public CustomOAuth2AuthenticationSuccessHandler successHandler() {
        return new CustomOAuth2AuthenticationSuccessHandler(userService);
    }

}
