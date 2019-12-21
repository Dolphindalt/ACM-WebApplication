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

  constructor(private http: HttpClient) {
    let user_data = JSON.parse(localStorage.getItem("currentUser"));
    if (user_data) {
      this.currentUserSubject = new BehaviorSubject<User>(user_data.user);
    } else {
      this.currentUserSubject = new BehaviorSubject<User>(null);
    }
    this.currentUser = this.currentUserSubject.asObservable();
  }

  public getCurrentUser(): User {
    return this.currentUserSubject.value;
  }

  public isAuthenticated(): boolean {
    return this.getCurrentUser() !== null;
  }

  login(email: string, password: string) {
    return this.http.post("http://localhost:8000/auth/login",
    {
      "email": email,
      "password": password
    }).pipe(
      map(data => {
        localStorage.setItem("currentUser", JSON.stringify(data["user"]));
        localStorage.setItem("access_token", JSON.stringify(data["token"]));
        return data["user"];
      })
    );
  }

  logout() {
    localStorage.clear();
    this.currentUserSubject.next(null);
  }
}
