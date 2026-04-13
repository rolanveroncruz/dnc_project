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
export interface PostHighEndVerificationApprovalRequest {
    approved_cost: string,
    dentist_notes: string | null,
}

export interface PostHighEndVerificationApprovalResponse {
    id: number,
    verification_id: number,
    approved_by: string | null,
    approved_cost: number | null,
    approval_date: string | null,
    dentist_notes: string | null,
    verification_status_id: number,
}

@Injectable({
  providedIn: 'root',
})
export class HighEndVerificationsService {
    constructor(
    private readonly http:HttpClient,
    private readonly loginService:LoginService){}


    private readonly baseHighEndVerificationsUrl = `${environment.apiBaseUrl}/api/high_end_verifications`;
    private readonly baseHighEndFilesUrl = `${environment.apiBaseUrl}/api/high_end_files`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }

    getAllHighEndVerifications(): Observable<HighEndVerificationResponse[]> {
        return this.http.get<HighEndVerificationResponse[]>(
            `${this.baseHighEndVerificationsUrl}`, { headers: this.authHeaders(), }
        );
    }
    downloadHighEndFile(fileId: number): Observable<Blob> {
        return this.http.get(
            `${this.baseHighEndFilesUrl}/${fileId}/download`,
            {
                headers: this.authHeaders(),
                responseType: 'blob',
            }
        );
    }


    postHighEndVerificationInformation(verification_id:number, dentist_notes: string|null, approved_cost: string):
        Observable<PostHighEndVerificationApprovalResponse> {
        const payload: PostHighEndVerificationApprovalRequest = {dentist_notes, approved_cost};
        return this.http.post<PostHighEndVerificationApprovalResponse>(
            `${this.baseHighEndVerificationsUrl}/${verification_id}/approval`,
            payload,
            {headers: this.authHeaders()}
        );
    }
}
