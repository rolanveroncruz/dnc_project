import { inject,Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../../environments/environment';

export interface SubmitDentistApplicationResponse {
    id: number;
    message: string;
}


@Injectable({
  providedIn: 'root',
})
export class HowToJoinService {
    private http = inject(HttpClient);
    private baseUrl = environment.apiUrl;


    submitDentistApplication(formData: FormData):Observable<SubmitDentistApplicationResponse>{
        return this.http.post<SubmitDentistApplicationResponse>(`${this.baseUrl}/public/dentist_applications`, formData);
    }
}
