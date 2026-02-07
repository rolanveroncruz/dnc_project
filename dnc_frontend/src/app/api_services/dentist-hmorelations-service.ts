// src/app/api_services/dentist-hmo-relations.service.ts
import { Injectable, inject } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import { Observable } from 'rxjs';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';

export interface HMOListItem{
    id: number;
    short_name: string;
}
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
    getExclusiveToHmos(dentistId: number): Observable<HMOListItem[]> {
        return this.http.get<HMOListItem []>(
            `${this.baseUrl}/dentists/${dentistId}/hmos/exclusive`, { headers: this.authHeaders()}
        );
    }
    addExclusiveToHmos(dentistId: number, hmoId: number): Observable<any> {
        return this.http.post<any>(
            `${this.baseUrl}/dentists/${dentistId}/hmos/exclusive/${encodeURIComponent(hmoId)}`, { hmo_id: hmoId }, { headers: this.authHeaders()}
        );
    }
    removeExclusiveToHmos(dentistId: number, hmoId: number): Observable<any> {
        console.log("removeExclusiveToHmos called");
        return this.http.delete<any>(
            `${this.baseUrl}/dentists/${dentistId}/hmos/exclusive/${encodeURIComponent(hmoId)}`, { headers: this.authHeaders()}
        );
    }

    /** GET /dentists/:dentist_id/hmos/not */
    getExceptForHmos(dentistId: number): Observable<HMOListItem[]> {
        return this.http.get<HMOListItem[]>(
            `${this.baseUrl}/dentists/${dentistId}/hmos/except`, { headers: this.authHeaders()}
        );
    }
    addExceptForHmos(dentistId: number, hmoId: number): Observable<any> {
        console.log("addExceptForHmos called");
        return this.http.post<any>(
            `${this.baseUrl}/dentists/${dentistId}/hmos/except/${encodeURIComponent(hmoId)}`, {hmo_id: hmoId},{ headers: this.authHeaders()}
        );
    }
    removeExceptForHmos(dentistId: number, hmoId: number): Observable<any> {
        return this.http.delete<any>(
            `${this.baseUrl}/dentists/${dentistId}/hmos/except/${encodeURIComponent(hmoId)}`, { headers: this.authHeaders()}
        );
    }
}
