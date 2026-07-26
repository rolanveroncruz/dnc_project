import {inject, Injectable} from '@angular/core';
import {
    HttpClient,
    HttpHeaders,
    HttpResponse,
} from '@angular/common/http';
import {Observable} from 'rxjs';

import {environment} from '../../../../environments/environment';
import {LoginService} from '../../../login.service';

export type DentistApplicationStatus =
    | 'new'
    | 'for_evaluation'
    | 'declined'
    | 'accredited';

export interface DentistApplicationRow {
    id: number;
    date_submitted: string | null;

    name: string;
    clinic_name: string;
    contact_numbers: string;
    email: string;

    clinic_ownership_type: string | null;
    hmo_affiliations: string | null;
    clinic_address: string | null;

    prc_license_file_path: string | null;
    bir_2303_file_path: string | null;
    registration_doc_file_path: string | null;
    supporting_docs_file_path1: string | null;

    status: string | null;
}

export interface UpdateDentistApplicationStatusResponse {
    id: number;
    status: string;
    message: string;
}

@Injectable({
    providedIn: 'root',
})
export class WebsiteApplicationsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);
    private readonly baseUrl = environment.apiUrl;

    getApplications(): Observable<DentistApplicationRow[]> {
        return this.http.get<DentistApplicationRow[]>(
            `${this.baseUrl}/api/website/dentist_applications`,
            {
                headers: this.authHeaders(),
            },
        );
    }

    updateStatus(
        applicationId: number,
        status: DentistApplicationStatus,
    ): Observable<UpdateDentistApplicationStatusResponse> {
        return this.http.patch<UpdateDentistApplicationStatusResponse>(
            `${this.baseUrl}/api/website/dentist_applications/${applicationId}/status`,
            {
                status,
            },
            {
                headers: this.authHeaders(),
            },
        );
    }

    downloadDocument(path: string): Observable<HttpResponse<Blob>> {
        return this.http.get(this.documentUrl(path), {
            headers: this.authHeaders(),
            responseType: 'blob',
            observe: 'response',
        });
    }

    private documentUrl(path: string): string {
        if (path.startsWith('http://') || path.startsWith('https://')) {
            return path;
        }

        if (path.startsWith('/')) {
            return `${this.baseUrl}${path}`;
        }

        return `${this.baseUrl}/${path}`;
    }

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';

        return new HttpHeaders({
            Authorization: `Bearer ${token}`,
        });
    }
}
