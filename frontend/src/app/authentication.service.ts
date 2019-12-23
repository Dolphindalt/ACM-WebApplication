import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject, Observable } from 'rxjs';
import { User } from './models/user';
import { map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class AuthenticationService {

  private currentUserSubject: BehaviorSubject<User>;
  public currentUser: Observable<User>;
  private authenticatedSubject: BehaviorSubject<boolean>;
  public authenticated: Observable<boolean>;

  constructor(private http: HttpClient) {
    let user_data = JSON.parse(localStorage.getItem("currentUser"));
    if (user_data) {
      this.currentUserSubject = new BehaviorSubject<User>(user_data.user);
      this.authenticatedSubject = new BehaviorSubject<boolean>(true);
    } else {
      this.currentUserSubject = new BehaviorSubject<User>(null);
      this.authenticatedSubject = new BehaviorSubject<boolean>(false);
    }
    this.currentUser = this.currentUserSubject.asObservable();
    this.authenticated = this.authenticatedSubject.asObservable();
  }

  public getCurrentUser(): User {
    return this.currentUserSubject.value;
  }

  public isAuthenticated(): boolean {
    return this.authenticatedSubject.value;
  }

  login(email: string, password: string) {
    return this.http.post("auth/login",
    {
      "email": email,
      "password": password
    }).pipe(
      map(data => {
        localStorage.setItem("currentUser", JSON.stringify(data["user"]));
        localStorage.setItem("access_token", JSON.stringify(data["token"]));
        this.currentUserSubject.next(JSON.parse(localStorage.getItem("currentUser")));
        this.authenticatedSubject.next(true);
        return data["user"];
      })
    );
  }

  logout() {
    localStorage.clear();
    this.currentUserSubject.next(null);
    this.authenticatedSubject.next(false);
  }
}
