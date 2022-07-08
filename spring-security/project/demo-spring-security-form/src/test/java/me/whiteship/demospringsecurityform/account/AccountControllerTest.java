package me.whiteship.demospringsecurityform.account;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.security.test.context.support.WithAnonymousUser;
import org.springframework.security.test.context.support.WithMockUser;
import org.springframework.test.context.junit4.SpringRunner;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.transaction.annotation.Transactional;

import static org.springframework.security.test.web.servlet.request.SecurityMockMvcRequestBuilders.formLogin;
import static org.springframework.security.test.web.servlet.request.SecurityMockMvcRequestPostProcessors.user;
import static org.springframework.security.test.web.servlet.response.SecurityMockMvcResultMatchers.authenticated;
import static org.springframework.security.test.web.servlet.response.SecurityMockMvcResultMatchers.unauthenticated;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.result.MockMvcResultHandlers.print;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

@RunWith(SpringRunner.class)
@SpringBootTest
@AutoConfigureMockMvc
public class AccountControllerTest {

    @Autowired
    MockMvc mockMvc;

//    @Test
//    public void index_anonymous() throws Exception {
//        mockMvc.perform(get("/").with(anonymous()))
//                .andDo(print())
//                .andExpect(status().isOk());
//    }

    @Test
    @WithAnonymousUser
    public void index_anonymous() throws Exception {
        mockMvc.perform(get("/"))
                .andDo(print())
                .andExpect(status().isOk());
    }

//    @Test
//    public void index_user() throws Exception {
//        mockMvc.perform(get("/").with(user("seungho").password("123").roles("USER")))   // 유저가 로그인을 한 상태라고 가정을 함 (DB에 있음을 의미하지 않음)
//                .andDo(print())
//                .andExpect(status().isOk());
//    }

    @Test
//    @WithMockUser(username = "seungho", roles = "USER")
    @WithUser
    public void index_user() throws Exception {
        mockMvc.perform(get("/"))
                .andDo(print())
                .andExpect(status().isOk());
    }

//    @Test
//    public void admin_user() throws Exception {
//        mockMvc.perform(get("/admin").with(user("seungho").password("123").roles("USER")))
//                .andDo(print())
//                .andExpect(status().isForbidden());
//    }

    @Test
//    @WithMockUser(username = "seungho", roles = "USER")
    @WithUser
    public void admin_user() throws Exception {
        mockMvc.perform(get("/admin"))
                .andDo(print())
                .andExpect(status().isForbidden());
    }

//    @Test
//    public void admin_admin() throws Exception {
//        mockMvc.perform(get("/admin").with(user("admin").password("123").roles("ADMIN")))
//                .andDo(print())
//                .andExpect(status().isOk());
//    }

    @Test
    @WithMockUser(username = "admin", roles = "ADMIN")
    public void admin_admin() throws Exception {
        mockMvc.perform(get("/admin").with(user("admin").password("123").roles("ADMIN")))
                .andDo(print())
                .andExpect(status().isOk());
    }

    @Autowired
    AccountService accountService;

    private void createUser(String username, String password) {
        Account account = new Account();
        account.setUsername(username);
        account.setPassword(password);
        account.setRole("USER");
        accountService.createNew(account);
    }

    @Test
    @Transactional  // 테스트 코드 실행 후 DB가 롤백됨
    public void login_success() throws Exception {
        String username = "seungho";
        String password = "123";
        createUser(username, password);
        mockMvc.perform(formLogin().user(username).password(password))
                .andExpect(authenticated());
    }

    @Test
    @Transactional
    public void login_fail() throws Exception {
        String username = "seungho";
        String password = "123";
        createUser(username, password);
        mockMvc.perform(formLogin().user(username).password("321"))
                .andExpect(unauthenticated());
    }

}
