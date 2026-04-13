import {Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';

export interface HighEndFileResponse {
    id: number,
    original_filename: string | null,
    description: string | null,
}

export interface HighEndVerificationResponse {
    verification_id: number,
    date_created: string,
    status_id: number,
    status_name: string,
    dentist_name: string,
    hmo_name: string,
    member_name: string,
    dental_service_name: string,
    files: HighEndFileResponse[],
}

@Injectable({
  providedIn: 'root',
})
export class HighEndVerificationsService {
    constructor(
    private readonly http:HttpClient,
    private readonly loginService:LoginService){}


    private readonly baseHighEndVerificationsUrl = `${environment.apiBaseUrl}/api/high_end_verifications`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }

    getAllHighEndVerifications(): Observable<HighEndVerificationResponse[]> {
        return this.http.get<HighEndVerificationResponse[]>(
            `${this.baseHighEndVerificationsUrl}`, { headers: this.authHeaders(), }
        );
    }
}
