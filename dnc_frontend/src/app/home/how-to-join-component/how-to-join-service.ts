import { inject,Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../../environments/environment';



@Injectable({
  providedIn: 'root',
})
export class HowToJoinService {
    private http = inject(HttpClient);
    private baseUrl = environment.apiUrl;


    submitDentistApplication(formData: FormData):Observable<void>{
        return this.http.post<void>(`${this.baseUrl}/public/dentist_applications`, formData);
    }
}
