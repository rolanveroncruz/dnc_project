// src/app/api_services/dentist-hmo-relations.service.ts
import { Injectable, inject } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import { Observable } from 'rxjs';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';

/**
 * Matches Rust handlers:
 *  - GET /dentists/:dentist_id/hmos/exclusive  -> Vec<String>
 *  - GET /dentists/:dentist_id/hmos/not        -> Vec<String>
 */
@Injectable({ providedIn: 'root' })
export class DentistHMORelationsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);


    // Adjust to your project style:
    // - if you have an ApiConfigService, inject that instead
    // - or replace with environment.apiBaseUrl
    private readonly baseUrl = `${environment.apiUrl}/api`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }
    /** GET /dentists/:dentist_id/hmos/exclusive */
    getExclusiveToHmos(dentistId: number): Observable<string[]> {
        return this.http.get<string[]>(
            `${this.baseUrl}/dentists/${dentistId}/hmos/exclusive`, { headers: this.authHeaders()}
        );
    }

    /** GET /dentists/:dentist_id/hmos/not */
    getExceptForHmos(dentistId: number): Observable<string[]> {
        return this.http.get<string[]>(
            `${this.baseUrl}/dentists/${dentistId}/hmos/except`, { headers: this.authHeaders()}
        );
    }
}
