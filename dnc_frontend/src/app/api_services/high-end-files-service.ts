import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';

export interface HighEndFileResponse {
    id: number;
    verification_id: number;
    filename: string;
}

@Injectable({
    providedIn: 'root',
})
export class HighEndFilesService {
    private http = inject(HttpClient);
    private loginService = inject(LoginService);


    private readonly baseHighEndFilesUrl = `${environment.apiBaseUrl}/api/high_end_files`;
    private readonly baseVerificationsUrl = `${environment.apiBaseUrl}/api/verifications`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }


    // upload a file for a given verification
    uploadHighEndFile(verificationId: number, file: File): Observable<HighEndFileResponse> {
        const formData = new FormData();
        formData.append('file', file);

        return this.http.post<HighEndFileResponse>(
            `${this.baseVerificationsUrl}/${verificationId}/upload`,
            formData,
            {
                headers: this.authHeaders(),
            }
        );
    }

    //  list files for a given verification
    getHighEndFilesByVerificationId(verificationId: number): Observable<HighEndFileResponse[]> {
        return this.http.get<HighEndFileResponse[]>(
            `${this.baseVerificationsUrl}/${verificationId}`,
            {
                headers: this.authHeaders(),
            }
        );
    }

    //  download a file by file id
    downloadHighEndFile(fileId: number): Observable<Blob> {
        return this.http.get(
            `${this.baseHighEndFilesUrl}/download/${fileId}`,
            {
                headers: this.authHeaders(),
                responseType: 'blob',
            }
        );
    }
}
