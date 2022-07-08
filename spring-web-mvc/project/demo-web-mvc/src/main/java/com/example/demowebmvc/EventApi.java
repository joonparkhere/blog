
package com.example.demowebmvc;

import org.springframework.http.HttpEntity;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.validation.BindingResult;
import org.springframework.web.bind.annotation.*;

import javax.validation.Valid;

@RestController
@RequestMapping("/api/events")
public class EventApi {

    @PostMapping
    public ResponseEntity<Event> createEvent(@RequestBody @Valid Event event, BindingResult bindingResult) {
        // save event
        if (bindingResult.hasErrors()) {
            return ResponseEntity.badRequest().build();
        }
        return ResponseEntity.ok(event);
    }
//    public Event createEvent(HttpEntity<Event> request) {
//        // save event
//        MediaType contentType = request.getHeaders().getContentType();
//        System.out.println(contentType);
//        return request.getBody();
//    }
//    public Event createEvent(@Valid @RequestBody Event event, BindingResult bindingResult) {
//        // save event
//        if (bindingResult.hasErrors()) {
//            bindingResult.getAllErrors().forEach(error -> {
//                System.out.println(error);
//            });
//        }
//        return event;
//    }

}
