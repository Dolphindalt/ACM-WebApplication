import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { MainNavComponent } from './main-nav/main-nav.component';
import { HomeComponent } from './home/home.component';
import { JwtModule } from "@auth0/angular-jwt";
import { HttpClientModule, HTTP_INTERCEPTORS } from "@angular/common/http";
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';
import { LoginComponent } from './login/login.component';
import { ToastsContainer } from './toast/toasts-container.component';
import { RegisterComponent } from './register/register.component';
import { environment } from 'src/environments/environment';
import { BaseurlService } from './baseurl.service';
import { EventComponent } from './event/event.component';
import { EventBoardComponent } from './event-board/event-board.component';
import { OfficerBoardComponent } from './officer-board/officer-board.component';
import { FileComponent } from './file/file.component';
import { UserComponent } from './user/user.component';

export function tokenGetter() {
  return localStorage.getItem("access_token");
}

@NgModule({
  declarations: [
    AppComponent,
    MainNavComponent,
    HomeComponent,
    LoginComponent,
    ToastsContainer,
    RegisterComponent,
    EventComponent,
    EventBoardComponent,
    OfficerBoardComponent,
    FileComponent,
    UserComponent
  ],
  imports: [
    BrowserModule,
    FontAwesomeModule,
    AppRoutingModule,
    HttpClientModule,
    JwtModule.forRoot({
      config: {
        tokenGetter: tokenGetter,
        whitelistedDomains: ["localhost:4200", "localhost:8000"]
      }
    }),
    FormsModule,
    ReactiveFormsModule,
    NgbModule
  ],
  providers: [
    { provide: HTTP_INTERCEPTORS, useClass: BaseurlService, multi: true },
    { provide: "BASE_API_URL", useValue: environment.apiUrl }
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
