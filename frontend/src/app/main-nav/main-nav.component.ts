import { Component, OnInit } from '@angular/core';
import { faBars } from '@fortawesome/free-solid-svg-icons';
import { AuthenticationService } from '../authentication.service';
import { Observable, Subscription } from 'rxjs';
 
@Component({
  selector: 'app-main-nav',
  templateUrl: './main-nav.component.html',
  styleUrls: ['./main-nav.component.sass']
})
export class MainNavComponent implements OnInit {

  faBars = faBars;
  private openedSubMenu: boolean = false;
  private authenticated: boolean = false;
  private subscription: Subscription;

  constructor(
    private authService: AuthenticationService
  ) {
    this.authenticated = authService.isAuthenticated();
    this.subscription = authService.authenticated.subscribe(
      (val) => {
        this.authenticated = val;
      }
    );
  }

  ngOnInit() {
    
  }

  toggleSubMenu() {
    this.openedSubMenu = !this.openedSubMenu;
  }

  attemptLogout() {
    this.authService.logout();
  }

}
